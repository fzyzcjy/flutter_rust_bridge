import React from 'react';

// noinspection JSUnusedGlobalSymbols
export default class FrbExampleGallery extends React.Component {
    componentDidMount() {
        console.log('FrbExampleGallery componentDidMount');

        window.addEventListener("load", function (ev) {
            console.log('FrbExampleGallery call flutter-loadEntrypoint');
            _flutter.loader.loadEntrypoint({
                onEntrypointLoaded: async function (engineInitializer) {
                    console.log('FrbExampleGallery call flutter-appRunner');
                    let appRunner = await engineInitializer.initializeEngine({
                        // Pass a reference to "div#flutter_host" into the Flutter engine.
                        hostElement: document.querySelector("#flutter_host")
                    });
                    await appRunner.runApp();
                }
            });
        });
    }

    render() {
        return (
            <div id="flutter_host" style={{height: '500px', border: '1px solid #ccc'}}>Loading Flutter web app...</div>
        )
    }
}
