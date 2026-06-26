#!/usr/bin/env bash
set -euo pipefail

flutter_root="${1:-}"
package_ohos_dir="${2:-}"

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

patch_count=0

if [[ -n "$flutter_root" ]]; then
  patch_helper "$flutter_root/engine/src/flutter/shell/platform/ohos/flutter_embedding/flutter/src/main/ets/plugin/editing/OhosAutoFillHelper.ets"
  patch_count=$((patch_count + 1))
fi

if [[ -n "$package_ohos_dir" ]]; then
  while IFS= read -r -d '' helper_path; do
    patch_helper "$helper_path"
    patch_count=$((patch_count + 1))
  done < <(find "$package_ohos_dir/oh_modules" -path '*/@ohos/flutter_ohos/src/main/ets/plugin/editing/OhosAutoFillHelper.ets' -print0)
fi

if [[ "$patch_count" -eq 0 ]]; then
  echo "No OHOS Flutter autofill helper was patched" >&2
  exit 1
fi
