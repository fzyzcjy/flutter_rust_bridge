// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="index.html">Introduction</a></span></li><li class="chapter-item expanded "><li class="part-title">Part I: Core</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="quickstart.html"><strong aria-hidden="true">1.</strong> 🧭 Quickstart</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorial_with_flutter.html"><strong aria-hidden="true">2.</strong> 📚 Tutorial: A Flutter+Rust app</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorial/setup_android.html"><strong aria-hidden="true">2.1.</strong> Android setup</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorial/alternative_ndk.html"><strong aria-hidden="true">2.1.1.</strong> Alternative NDK setup</a></span></li></ol></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature.html"><strong aria-hidden="true">3.</strong> 🎼 Features</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang.html"><strong aria-hidden="true">3.1.</strong> Language translations</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang_simple.html"><strong aria-hidden="true">3.1.1.</strong> Simple correspondence</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang_vec.html"><strong aria-hidden="true">3.1.2.</strong> Vec and array</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang_struct.html"><strong aria-hidden="true">3.1.3.</strong> Struct</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang_enum.html"><strong aria-hidden="true">3.1.4.</strong> Enum</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang_tuple.html"><strong aria-hidden="true">3.1.5.</strong> Tuples</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang_external.html"><strong aria-hidden="true">3.1.6.</strong> External types</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang_option.html"><strong aria-hidden="true">3.1.7.</strong> Option</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang_methods.html"><strong aria-hidden="true">3.1.8.</strong> Methods</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang_return_types.html"><strong aria-hidden="true">3.1.9.</strong> Return types</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang_dynamic.html"><strong aria-hidden="true">3.1.10.</strong> Dynamic</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang_rust_opaque.html"><strong aria-hidden="true">3.1.11.</strong> Arbitrary Rust types (opaque)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang_dart_opaque.html"><strong aria-hidden="true">3.1.12.</strong> Arbitrary Dart types (opaque)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang_type_alias.html"><strong aria-hidden="true">3.1.13.</strong> Type alias</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang_exceptions.html"><strong aria-hidden="true">3.1.14.</strong> Result / Exceptions</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/lang_default.html"><strong aria-hidden="true">3.1.15.</strong> Parameter defaults</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/zero_copy.html"><strong aria-hidden="true">3.2.</strong> Zero copy</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/stream.html"><strong aria-hidden="true">3.3.</strong> Stream / Iterator</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/async_dart.html"><strong aria-hidden="true">3.4.</strong> Async in Dart</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/sync_dart.html"><strong aria-hidden="true">3.5.</strong> Sync in Dart</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/concurrency.html"><strong aria-hidden="true">3.6.</strong> Concurrency</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/handler.html"><strong aria-hidden="true">3.7.</strong> Handler</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/init.html"><strong aria-hidden="true">3.8.</strong> Initialization</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/async_rust.html"><strong aria-hidden="true">3.9.</strong> Async in Rust</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/multiple_files.html"><strong aria-hidden="true">3.10.</strong> Multiple files</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/build_rs.html"><strong aria-hidden="true">3.11.</strong> Run in build.rs</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/cancelable_task.html"><strong aria-hidden="true">3.12.</strong> Cancellable tasks</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/object_pool.html"><strong aria-hidden="true">3.13.</strong> Object pools</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/wasm.html"><strong aria-hidden="true">3.14.</strong> WASM</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/misc.html"><strong aria-hidden="true">3.15.</strong> Miscellaneous</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/logging.html"><strong aria-hidden="true">3.16.</strong> Logging</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/stack_trace.html"><strong aria-hidden="true">3.17.</strong> Stack Traces</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/worker_pool.html"><strong aria-hidden="true">3.18.</strong> Worker pool</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="feature/expanding_macros.html"><strong aria-hidden="true">3.19.</strong> Expanding macros</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Part II: User Guide</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template.html"><strong aria-hidden="true">4.</strong> Create new projects from a template</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/setup.html"><strong aria-hidden="true">4.1.</strong> Creating a new project</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/setup_android.html"><strong aria-hidden="true">4.1.1.</strong> Android setup</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/setup_ios.html"><strong aria-hidden="true">4.1.2.</strong> iOS setup</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/setup_web.html"><strong aria-hidden="true">4.1.3.</strong> Web setup</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/setup_desktop.html"><strong aria-hidden="true">4.1.4.</strong> Windows and Linux</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/setup_others.html"><strong aria-hidden="true">4.1.5.</strong> Other platforms</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/tour.html"><strong aria-hidden="true">4.2.</strong> Template tour</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/tour_api.html"><strong aria-hidden="true">4.2.1.</strong> native/src/api.rs</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/tour_gradle.html"><strong aria-hidden="true">4.2.2.</strong> android/app/build.gradle</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/tour_native_proj.html"><strong aria-hidden="true">4.2.3.</strong> native/native.xcodeproj</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/tour_justfile.html"><strong aria-hidden="true">4.2.4.</strong> justfile</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/tour_cmake.html"><strong aria-hidden="true">4.2.5.</strong> rust.cmake</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/generate.html"><strong aria-hidden="true">4.3.</strong> Generating code</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/generate_install.html"><strong aria-hidden="true">4.3.1.</strong> Installing codegen</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/generate_adding_code.html"><strong aria-hidden="true">4.3.2.</strong> Adding new code</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/generate_build_runner.html"><strong aria-hidden="true">4.3.3.</strong> Using build_runner</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="template/generate_finish.html"><strong aria-hidden="true">4.3.4.</strong> Wrapping up</a></span></li></ol></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="integrate.html"><strong aria-hidden="true">5.</strong> Integrating with existing projects</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="integrate/new_crate.html"><strong aria-hidden="true">5.1.</strong> Creating a new crate</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="integrate/deps.html"><strong aria-hidden="true">5.2.</strong> Installing dependencies</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="integrate/android.html"><strong aria-hidden="true">5.3.</strong> Integrating with Android</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="integrate/android_tasks.html"><strong aria-hidden="true">5.3.1.</strong> Hooking onto tasks</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="integrate/android_cmake.html"><strong aria-hidden="true">5.3.2.</strong> CMake with Gradle</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="integrate/ios.html"><strong aria-hidden="true">5.4.</strong> Integrating with iOS/MacOS</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="integrate/ios_proj.html"><strong aria-hidden="true">5.4.1.</strong> Creating the Rust project</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="integrate/ios_linking.html"><strong aria-hidden="true">5.4.2.</strong> Linking the project</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="integrate/ios_gen.html"><strong aria-hidden="true">5.4.3.</strong> Generating bindings</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="integrate/ios_headers.html"><strong aria-hidden="true">5.4.4.</strong> Using dummy headers</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="integrate/desktop.html"><strong aria-hidden="true">5.5.</strong> Integrating with Windows and Linux</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="integrate/web.html"><strong aria-hidden="true">5.6.</strong> Integrating with Web</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="integrate/usage.html"><strong aria-hidden="true">5.7.</strong> Using the dynamic library</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="integrate/finish.html"><strong aria-hidden="true">5.8.</strong> Wrapping up</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="library.html"><strong aria-hidden="true">6.</strong> Creating a Dart/Flutter library</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="library/overview.html"><strong aria-hidden="true">6.1.</strong> Overview</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="library/setup.html"><strong aria-hidden="true">6.1.1.</strong> Setup</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="library/melos.html"><strong aria-hidden="true">6.1.2.</strong> Monorepo with Melos</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="library/creating_libraries.html"><strong aria-hidden="true">6.2.</strong> Creating the libraries</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="library/dart_only.html"><strong aria-hidden="true">6.2.1.</strong> Dart-only base</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="library/flutter_wrapper.html"><strong aria-hidden="true">6.2.2.</strong> Flutter wrapper</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="library/platform_setup.html"><strong aria-hidden="true">6.3.</strong> Platform specific setup</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="library/platform_setup/windows_and_linux.html"><strong aria-hidden="true">6.3.1.</strong> Windows &amp; Linux</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="library/platform_setup/ios_and_macos.html"><strong aria-hidden="true">6.3.2.</strong> iOS &amp; macOS</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="library/platform_setup/android.html"><strong aria-hidden="true">6.3.3.</strong> Android</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="library/ci.html"><strong aria-hidden="true">6.4.</strong> Continuous Integration &amp; Deployment (CI/CD)</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Part III: Contributor Guide</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/overview.html"><strong aria-hidden="true">7.</strong> Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/design.html"><strong aria-hidden="true">8.</strong> Overall design</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/submodule.html"><strong aria-hidden="true">9.</strong> Submodule implementations</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/rust_opaque_type_safety.html"><strong aria-hidden="true">9.1.</strong> Rust opaque type safety</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/dart_opaque_type_safety.html"><strong aria-hidden="true">9.2.</strong> Dart opaque type safety</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="contributing/appendix.html"><strong aria-hidden="true">10.</strong> Appendix</a></span></li><li class="chapter-item expanded "><li class="part-title">Part IV: More Doc</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="unit_tests_dart.html"><strong aria-hidden="true">11.</strong> Unit testing in Flutter</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorial_pure_dart.html"><strong aria-hidden="true">12.</strong> Tutorial: Pure Dart</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="safety.html"><strong aria-hidden="true">13.</strong> Safety concerns</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="troubleshooting.html"><strong aria-hidden="true">14.</strong> Troubleshooting</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="command_line.html"><strong aria-hidden="true">15.</strong> Command line arguments</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="set_up_from_scratch.html"><strong aria-hidden="true">16.</strong> Set up Flutter/Dart+Rust support from scratch</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="build_wasm.html"><strong aria-hidden="true">17.</strong> Building a WASM binary manually</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="wasm_limitations.html"><strong aria-hidden="true">18.</strong> Limitations of WASM</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="article.html"><strong aria-hidden="true">19.</strong> Articles</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="article/async_in_rust.html"><strong aria-hidden="true">19.1.</strong> Async in Rust</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="article/generate_multiple_files.html"><strong aria-hidden="true">19.2.</strong> Generate multiple files</a></span></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split('#')[0].split('?')[0];
        if (current_page.endsWith('/')) {
            current_page += 'index.html';
        }
        const links = Array.prototype.slice.call(this.querySelectorAll('a'));
        const l = links.length;
        for (let i = 0; i < l; ++i) {
            const link = links[i];
            const href = link.getAttribute('href');
            if (href && !href.startsWith('#') && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The 'index' page is supposed to alias the first chapter in the book.
            if (link.href === current_page
                || i === 0
                && path_to_root === ''
                && current_page.endsWith('/index.html')) {
                link.classList.add('active');
                let parent = link.parentElement;
                while (parent) {
                    if (parent.tagName === 'LI' && parent.classList.contains('chapter-item')) {
                        parent.classList.add('expanded');
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', e => {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        const sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via
            // 'next/previous chapter' buttons
            const activeSection = document.querySelector('#mdbook-sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        const sidebarAnchorToggles = document.querySelectorAll('.chapter-fold-toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(el => {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define('mdbook-sidebar-scrollbox', MDBookSidebarScrollbox);


// ---------------------------------------------------------------------------
// Support for dynamically adding headers to the sidebar.

(function() {
    // This is used to detect which direction the page has scrolled since the
    // last scroll event.
    let lastKnownScrollPosition = 0;
    // This is the threshold in px from the top of the screen where it will
    // consider a header the "current" header when scrolling down.
    const defaultDownThreshold = 150;
    // Same as defaultDownThreshold, except when scrolling up.
    const defaultUpThreshold = 300;
    // The threshold is a virtual horizontal line on the screen where it
    // considers the "current" header to be above the line. The threshold is
    // modified dynamically to handle headers that are near the bottom of the
    // screen, and to slightly offset the behavior when scrolling up vs down.
    let threshold = defaultDownThreshold;
    // This is used to disable updates while scrolling. This is needed when
    // clicking the header in the sidebar, which triggers a scroll event. It
    // is somewhat finicky to detect when the scroll has finished, so this
    // uses a relatively dumb system of disabling scroll updates for a short
    // time after the click.
    let disableScroll = false;
    // Array of header elements on the page.
    let headers;
    // Array of li elements that are initially collapsed headers in the sidebar.
    // I'm not sure why eslint seems to have a false positive here.
    // eslint-disable-next-line prefer-const
    let headerToggles = [];
    // This is a debugging tool for the threshold which you can enable in the console.
    let thresholdDebug = false;

    // Updates the threshold based on the scroll position.
    function updateThreshold() {
        const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
        const windowHeight = window.innerHeight;
        const documentHeight = document.documentElement.scrollHeight;

        // The number of pixels below the viewport, at most documentHeight.
        // This is used to push the threshold down to the bottom of the page
        // as the user scrolls towards the bottom.
        const pixelsBelow = Math.max(0, documentHeight - (scrollTop + windowHeight));
        // The number of pixels above the viewport, at least defaultDownThreshold.
        // Similar to pixelsBelow, this is used to push the threshold back towards
        // the top when reaching the top of the page.
        const pixelsAbove = Math.max(0, defaultDownThreshold - scrollTop);
        // How much the threshold should be offset once it gets close to the
        // bottom of the page.
        const bottomAdd = Math.max(0, windowHeight - pixelsBelow - defaultDownThreshold);
        let adjustedBottomAdd = bottomAdd;

        // Adjusts bottomAdd for a small document. The calculation above
        // assumes the document is at least twice the windowheight in size. If
        // it is less than that, then bottomAdd needs to be shrunk
        // proportional to the difference in size.
        if (documentHeight < windowHeight * 2) {
            const maxPixelsBelow = documentHeight - windowHeight;
            const t = 1 - pixelsBelow / Math.max(1, maxPixelsBelow);
            const clamp = Math.max(0, Math.min(1, t));
            adjustedBottomAdd *= clamp;
        }

        let scrollingDown = true;
        if (scrollTop < lastKnownScrollPosition) {
            scrollingDown = false;
        }

        if (scrollingDown) {
            // When scrolling down, move the threshold up towards the default
            // downwards threshold position. If near the bottom of the page,
            // adjustedBottomAdd will offset the threshold towards the bottom
            // of the page.
            const amountScrolledDown = scrollTop - lastKnownScrollPosition;
            const adjustedDefault = defaultDownThreshold + adjustedBottomAdd;
            threshold = Math.max(adjustedDefault, threshold - amountScrolledDown);
        } else {
            // When scrolling up, move the threshold down towards the default
            // upwards threshold position. If near the bottom of the page,
            // quickly transition the threshold back up where it normally
            // belongs.
            const amountScrolledUp = lastKnownScrollPosition - scrollTop;
            const adjustedDefault = defaultUpThreshold - pixelsAbove
                + Math.max(0, adjustedBottomAdd - defaultDownThreshold);
            threshold = Math.min(adjustedDefault, threshold + amountScrolledUp);
        }

        if (documentHeight <= windowHeight) {
            threshold = 0;
        }

        if (thresholdDebug) {
            const id = 'mdbook-threshold-debug-data';
            let data = document.getElementById(id);
            if (data === null) {
                data = document.createElement('div');
                data.id = id;
                data.style.cssText = `
                    position: fixed;
                    top: 50px;
                    right: 10px;
                    background-color: 0xeeeeee;
                    z-index: 9999;
                    pointer-events: none;
                `;
                document.body.appendChild(data);
            }
            data.innerHTML = `
                <table>
                  <tr><td>documentHeight</td><td>${documentHeight.toFixed(1)}</td></tr>
                  <tr><td>windowHeight</td><td>${windowHeight.toFixed(1)}</td></tr>
                  <tr><td>scrollTop</td><td>${scrollTop.toFixed(1)}</td></tr>
                  <tr><td>pixelsAbove</td><td>${pixelsAbove.toFixed(1)}</td></tr>
                  <tr><td>pixelsBelow</td><td>${pixelsBelow.toFixed(1)}</td></tr>
                  <tr><td>bottomAdd</td><td>${bottomAdd.toFixed(1)}</td></tr>
                  <tr><td>adjustedBottomAdd</td><td>${adjustedBottomAdd.toFixed(1)}</td></tr>
                  <tr><td>scrollingDown</td><td>${scrollingDown}</td></tr>
                  <tr><td>threshold</td><td>${threshold.toFixed(1)}</td></tr>
                </table>
            `;
            drawDebugLine();
        }

        lastKnownScrollPosition = scrollTop;
    }

    function drawDebugLine() {
        if (!document.body) {
            return;
        }
        const id = 'mdbook-threshold-debug-line';
        const existingLine = document.getElementById(id);
        if (existingLine) {
            existingLine.remove();
        }
        const line = document.createElement('div');
        line.id = id;
        line.style.cssText = `
            position: fixed;
            top: ${threshold}px;
            left: 0;
            width: 100vw;
            height: 2px;
            background-color: red;
            z-index: 9999;
            pointer-events: none;
        `;
        document.body.appendChild(line);
    }

    function mdbookEnableThresholdDebug() {
        thresholdDebug = true;
        updateThreshold();
        drawDebugLine();
    }

    window.mdbookEnableThresholdDebug = mdbookEnableThresholdDebug;

    // Updates which headers in the sidebar should be expanded. If the current
    // header is inside a collapsed group, then it, and all its parents should
    // be expanded.
    function updateHeaderExpanded(currentA) {
        // Add expanded to all header-item li ancestors.
        let current = currentA.parentElement;
        while (current) {
            if (current.tagName === 'LI' && current.classList.contains('header-item')) {
                current.classList.add('expanded');
            }
            current = current.parentElement;
        }
    }

    // Updates which header is marked as the "current" header in the sidebar.
    // This is done with a virtual Y threshold, where headers at or below
    // that line will be considered the current one.
    function updateCurrentHeader() {
        if (!headers || !headers.length) {
            return;
        }

        // Reset the classes, which will be rebuilt below.
        const els = document.getElementsByClassName('current-header');
        for (const el of els) {
            el.classList.remove('current-header');
        }
        for (const toggle of headerToggles) {
            toggle.classList.remove('expanded');
        }

        // Find the last header that is above the threshold.
        let lastHeader = null;
        for (const header of headers) {
            const rect = header.getBoundingClientRect();
            if (rect.top <= threshold) {
                lastHeader = header;
            } else {
                break;
            }
        }
        if (lastHeader === null) {
            lastHeader = headers[0];
            const rect = lastHeader.getBoundingClientRect();
            const windowHeight = window.innerHeight;
            if (rect.top >= windowHeight) {
                return;
            }
        }

        // Get the anchor in the summary.
        const href = '#' + lastHeader.id;
        const a = [...document.querySelectorAll('.header-in-summary')]
            .find(element => element.getAttribute('href') === href);
        if (!a) {
            return;
        }

        a.classList.add('current-header');

        updateHeaderExpanded(a);
    }

    // Updates which header is "current" based on the threshold line.
    function reloadCurrentHeader() {
        if (disableScroll) {
            return;
        }
        updateThreshold();
        updateCurrentHeader();
    }


    // When clicking on a header in the sidebar, this adjusts the threshold so
    // that it is located next to the header. This is so that header becomes
    // "current".
    function headerThresholdClick(event) {
        // See disableScroll description why this is done.
        disableScroll = true;
        setTimeout(() => {
            disableScroll = false;
        }, 100);
        // requestAnimationFrame is used to delay the update of the "current"
        // header until after the scroll is done, and the header is in the new
        // position.
        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                // Closest is needed because if it has child elements like <code>.
                const a = event.target.closest('a');
                const href = a.getAttribute('href');
                const targetId = href.substring(1);
                const targetElement = document.getElementById(targetId);
                if (targetElement) {
                    threshold = targetElement.getBoundingClientRect().bottom;
                    updateCurrentHeader();
                }
            });
        });
    }

    // Takes the nodes from the given head and copies them over to the
    // destination, along with some filtering.
    function filterHeader(source, dest) {
        const clone = source.cloneNode(true);
        clone.querySelectorAll('mark').forEach(mark => {
            mark.replaceWith(...mark.childNodes);
        });
        dest.append(...clone.childNodes);
    }

    // Scans page for headers and adds them to the sidebar.
    document.addEventListener('DOMContentLoaded', function() {
        const activeSection = document.querySelector('#mdbook-sidebar .active');
        if (activeSection === null) {
            return;
        }

        const main = document.getElementsByTagName('main')[0];
        headers = Array.from(main.querySelectorAll('h2, h3, h4, h5, h6'))
            .filter(h => h.id !== '' && h.children.length && h.children[0].tagName === 'A');

        if (headers.length === 0) {
            return;
        }

        // Build a tree of headers in the sidebar.

        const stack = [];

        const firstLevel = parseInt(headers[0].tagName.charAt(1));
        for (let i = 1; i < firstLevel; i++) {
            const ol = document.createElement('ol');
            ol.classList.add('section');
            if (stack.length > 0) {
                stack[stack.length - 1].ol.appendChild(ol);
            }
            stack.push({level: i + 1, ol: ol});
        }

        // The level where it will start folding deeply nested headers.
        const foldLevel = 3;

        for (let i = 0; i < headers.length; i++) {
            const header = headers[i];
            const level = parseInt(header.tagName.charAt(1));

            const currentLevel = stack[stack.length - 1].level;
            if (level > currentLevel) {
                // Begin nesting to this level.
                for (let nextLevel = currentLevel + 1; nextLevel <= level; nextLevel++) {
                    const ol = document.createElement('ol');
                    ol.classList.add('section');
                    const last = stack[stack.length - 1];
                    const lastChild = last.ol.lastChild;
                    // Handle the case where jumping more than one nesting
                    // level, which doesn't have a list item to place this new
                    // list inside of.
                    if (lastChild) {
                        lastChild.appendChild(ol);
                    } else {
                        last.ol.appendChild(ol);
                    }
                    stack.push({level: nextLevel, ol: ol});
                }
            } else if (level < currentLevel) {
                while (stack.length > 1 && stack[stack.length - 1].level > level) {
                    stack.pop();
                }
            }

            const li = document.createElement('li');
            li.classList.add('header-item');
            li.classList.add('expanded');
            if (level < foldLevel) {
                li.classList.add('expanded');
            }
            const span = document.createElement('span');
            span.classList.add('chapter-link-wrapper');
            const a = document.createElement('a');
            span.appendChild(a);
            a.href = '#' + header.id;
            a.classList.add('header-in-summary');
            filterHeader(header.children[0], a);
            a.addEventListener('click', headerThresholdClick);
            const nextHeader = headers[i + 1];
            if (nextHeader !== undefined) {
                const nextLevel = parseInt(nextHeader.tagName.charAt(1));
                if (nextLevel > level && level >= foldLevel) {
                    const toggle = document.createElement('a');
                    toggle.classList.add('chapter-fold-toggle');
                    toggle.classList.add('header-toggle');
                    toggle.addEventListener('click', () => {
                        li.classList.toggle('expanded');
                    });
                    const toggleDiv = document.createElement('div');
                    toggleDiv.textContent = '❱';
                    toggle.appendChild(toggleDiv);
                    span.appendChild(toggle);
                    headerToggles.push(li);
                }
            }
            li.appendChild(span);

            const currentParent = stack[stack.length - 1];
            currentParent.ol.appendChild(li);
        }

        const onThisPage = document.createElement('div');
        onThisPage.classList.add('on-this-page');
        onThisPage.append(stack[0].ol);
        const activeItemSpan = activeSection.parentElement;
        activeItemSpan.after(onThisPage);
    });

    document.addEventListener('DOMContentLoaded', reloadCurrentHeader);
    document.addEventListener('scroll', reloadCurrentHeader, { passive: true });
})();

