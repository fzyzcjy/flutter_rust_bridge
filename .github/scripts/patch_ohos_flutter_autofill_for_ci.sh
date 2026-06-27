#!/usr/bin/env bash
set -euo pipefail

flutter_root="${1:-}"
package_ohos_dir="${2:-}"

patch_text_if_present() {
  local file_path="$1"
  local pattern="$2"
  local replacement="$3"
  perl -0pi -e "s/$pattern/$replacement/g" "$file_path"
}

patch_helper() {
  local helper_path="$1"
  test -f "$helper_path"
  cat > "$helper_path" <<'EOF'
import { Autofill, Configuration } from '../../embedding/engine/systemchannels/TextInputChannel';
import common from '@ohos.app.ability.common';

export const AUTOFILL_SUPPORT_API = 999;

export interface AutoFillFieldRect {
  left: number;
  top: number;
  width: number;
  height: number;
}

export interface AutoFillHostInfo {
  bundleName: string;
  moduleName: string;
  abilityName: string;
  pageUrl: string;
}

export interface OhosAutoFillRect {
  left: number;
  top: number;
  width: number;
  height: number;
}

export interface OhosPageNodeInfo {
  id: number;
  depth: number;
  autoFillType: number;
  tag: string;
  value: string;
  placeholder: string;
  enableAutoFill: boolean;
  rect: OhosAutoFillRect;
  isFocus: boolean;
}

export interface OhosViewData {
  bundleName: string;
  moduleName: string;
  abilityName: string;
  pageUrl: string;
  pageNodeInfos: OhosPageNodeInfo[];
  pageRect: OhosAutoFillRect;
  isUserSelected: boolean;
  isOtherAccount: boolean;
}

interface AutoSaveCallback {
  onSuccess(): void;
  onFailure(): void;
}

export class OhosAutoFillHelper {
  static logViewDataSummary(viewData: OhosViewData, label: string): void {}

  static setHostPageUrl(pageUrl: string): void {}

  static setHostAbilityContext(context: common.UIAbilityContext): void {}

  static getCachedHostInfo(): AutoFillHostInfo | null {
    return null;
  }

  static async loadHostInfo(): Promise<AutoFillHostInfo> {
    return {
      bundleName: '',
      moduleName: 'entry',
      abilityName: 'EntryAbility',
      pageUrl: 'pages/Index',
    };
  }

  static hintToAutoFillType(hint: string): number {
    return 0;
  }

  static primaryAutoFillType(autofill: Autofill): number {
    return 0;
  }

  static isValidRect(rect: AutoFillFieldRect | null): boolean {
    return rect !== null && rect.width > 0 && rect.height > 0;
  }

  static buildNodeIdMap(configuration: Configuration): Map<string, number> {
    const map = new Map<string, number>();
    const fields = configuration.fields;
    let counter = 0;
    if (fields !== null) {
      for (const field of fields) {
        const autofill = field.autofill;
        if (autofill !== undefined && !map.has(autofill.uniqueIdentifier)) {
          map.set(autofill.uniqueIdentifier, counter++);
        }
      }
    }
    const autofill = configuration.autofill;
    if (autofill !== undefined && !map.has(autofill.uniqueIdentifier)) {
      map.set(autofill.uniqueIdentifier, counter);
    }
    return map;
  }

  static buildViewData(
    hostInfo: AutoFillHostInfo,
    configuration: Configuration,
    nodeIdMap: Map<string, number>,
    focusedFieldRect: AutoFillFieldRect | null,
    pageRect: AutoFillFieldRect | null,
    fieldRects?: Map<string, AutoFillFieldRect>,
  ): OhosViewData {
    return OhosAutoFillHelper.emptyViewData(hostInfo);
  }

  static buildViewDataForSave(
    hostInfo: AutoFillHostInfo,
    fieldConfigs: Map<string, Configuration>,
    fieldTexts: Map<string, string>,
    nodeIdMap: Map<string, number>,
    focusedUniqueId: string,
    focusedFieldRect: AutoFillFieldRect | null,
    pageRect: AutoFillFieldRect | null,
    fieldRects?: Map<string, AutoFillFieldRect>,
  ): OhosViewData {
    return OhosAutoFillHelper.emptyViewData(hostInfo);
  }

  static requestAutoFill(
    uiContext: UIContext,
    viewData: OhosViewData,
    autoFillType: number,
    onSuccess: (values: Map<number, string>) => void,
    onFailure: (errCode: number) => void,
  ): void {
    onFailure(-1);
  }

  static requestAutoSave(
    uiContext: UIContext,
    viewData: OhosViewData,
    napiCallback: AutoSaveCallback,
  ): void {
    napiCallback.onFailure();
  }

  private static emptyViewData(hostInfo: AutoFillHostInfo): OhosViewData {
    return {
      bundleName: hostInfo.bundleName,
      moduleName: hostInfo.moduleName,
      abilityName: hostInfo.abilityName,
      pageUrl: hostInfo.pageUrl,
      pageNodeInfos: [],
      pageRect: { left: 0, top: 0, width: 1, height: 1 },
      isUserSelected: false,
      isOtherAccount: false,
    };
  }
}
EOF
}

patch_key_event_handler() {
  local handler_path="$1"
  test -f "$handler_path"
  patch_text_if_present \
    "$handler_path" \
    'const isCapsLockOn = event\.isCapsLockOn !== undefined \? event\.isCapsLockOn : false;' \
    'const isCapsLockOn = false;'
  patch_text_if_present \
    "$handler_path" \
    'const isNumLockOn = event\.isNumLockOn !== undefined \? event\.isNumLockOn : true;' \
    'const isNumLockOn = true;'
}

patch_pip_visibility_bridge() {
  local bridge_path="$1"
  test -f "$bridge_path"
  perl -0pi -e 's/private async queryPipMode\(\): Promise<boolean> \{\n    if \(this\.shownCount > 0\) \{\n      return false;\n    \}\n\n    try \{\n      const mode = await window\.getGlobalWindowMode\(\);\n      return \(mode & window\.GlobalWindowMode\.PIP\) !== 0;\n    \} catch \(error\) \{\n      Log\.w\(TAG, "getGlobalWindowMode failed: " \+ JSON\.stringify\(error\)\);\n      return false;\n    \}\n  \}/private async queryPipMode(): Promise<boolean> {\n    return false;\n  \}/g' "$bridge_path"
}

patch_embedding_root() {
  local embedding_root="$1"
  patch_helper "$embedding_root/plugin/editing/OhosAutoFillHelper.ets"
  patch_key_event_handler "$embedding_root/embedding/ohos/KeyEventHandler.ets"
  patch_pip_visibility_bridge "$embedding_root/embedding/ohos/PiPVisibilityBridge.ets"
  echo "Patched OHOS Flutter SDK 5.1 compatibility files under $embedding_root"
}

patch_har() {
  local har_path="$1"
  local temp_dir
  temp_dir="$(mktemp -d)"
  local har_format
  if unzip -tq "$har_path" >/dev/null 2>&1; then
    har_format="zip"
    unzip -q "$har_path" -d "$temp_dir"
  elif gzip -t "$har_path" >/dev/null 2>&1 && gzip -dc "$har_path" | tar -tf - >/dev/null 2>&1; then
    har_format="tar_gzip"
    gzip -dc "$har_path" | tar -xf - -C "$temp_dir"
  else
    echo "Unsupported OHOS Flutter HAR format: $har_path" >&2
    rm -rf "$temp_dir"
    exit 1
  fi
  local har_patch_count=0
  while IFS= read -r -d '' embedding_root; do
    patch_embedding_root "$embedding_root"
    har_patch_count=$((har_patch_count + 1))
  done < <(find "$temp_dir" -path '*/src/main/ets' -type d -print0)
  if [[ "$har_patch_count" -eq 0 ]]; then
    echo "No OHOS Flutter embedding root was found in $har_path" >&2
    rm -rf "$temp_dir"
    exit 1
  fi
  local temp_har
  temp_har="$(mktemp)"
  rm -f "$temp_har"
  if [[ "$har_format" == "zip" ]]; then
    (
      cd "$temp_dir"
      zip -qr "$temp_har" .
    )
  else
    (
      cd "$temp_dir"
      if [[ -e package ]]; then
        tar -cf - package | gzip -n > "$temp_har"
      else
        shopt -s nullglob dotglob
        entries=(*)
        if [[ "${#entries[@]}" -eq 0 ]]; then
          echo "No files found while repacking OHOS Flutter HAR: $har_path" >&2
          exit 1
        fi
        tar -cf - -- "${entries[@]}" | gzip -n > "$temp_har"
      fi
    )
  fi
  mv "$temp_har" "$har_path"
  rm -rf "$temp_dir"
  echo "Patched OHOS Flutter HAR $har_path"
}

patch_flutter_hars() {
  local flutter_root="$1"
  local engine_cache_dir="$flutter_root/bin/cache/artifacts/engine"
  if [[ ! -d "$engine_cache_dir" ]]; then
    echo "OHOS Flutter engine cache does not exist yet: $engine_cache_dir"
    return
  fi
  local har_count=0
  while IFS= read -r -d '' har_path; do
    patch_har "$har_path"
    har_count=$((har_count + 1))
  done < <(find "$engine_cache_dir" -path '*/flutter_embedding_*.har' -type f -print0)
  if [[ "$har_count" -eq 0 ]]; then
    echo "No OHOS Flutter embedding HAR was patched under $engine_cache_dir"
  fi
}

patch_count=0

if [[ -n "$flutter_root" ]]; then
  patch_embedding_root "$flutter_root/engine/src/flutter/shell/platform/ohos/flutter_embedding/flutter/src/main/ets"
  patch_flutter_hars "$flutter_root"
  patch_count=$((patch_count + 1))
fi

if [[ -n "$package_ohos_dir" ]]; then
  package_patch_count=0
  while IFS= read -r -d '' embedding_root; do
    patch_embedding_root "$embedding_root"
    package_patch_count=$((package_patch_count + 1))
    patch_count=$((patch_count + 1))
  done < <(find "$package_ohos_dir/oh_modules" -path '*/@ohos/flutter_ohos/src/main/ets' -type d -print0)
  if [[ "$package_patch_count" -eq 0 ]]; then
    echo "No OHOS Flutter package embedding root was patched under $package_ohos_dir/oh_modules"
  fi
fi

if [[ "$patch_count" -eq 0 ]]; then
  echo "No OHOS Flutter autofill helper was patched" >&2
  exit 1
fi
