import React from 'react';

// noinspection JSUnusedGlobalSymbols
export default function FrbExampleGallery() {
    return (
        <div style={{position: 'relative', marginBottom: '12px'}}>
            <div id="flutter_host" style={{height: '500px', border: '1px solid #ccc'}}></div>
            <FlutterInitializer/>
        </div>
    )
}

class FlutterInitializer extends React.Component {
    state = {
        loading: true,
    };

    componentDidMount = () => {
        console.log('FrbExampleGallery.FlutterInitializer componentDidMount');
        const that = this;

        {
            console.log('FrbExampleGallery.FlutterInitializer add enable-threads.js script');
            const script = document.createElement("script");
            script.src = '/flutter_rust_bridge/demo/enable-threads.js';
            document.body.appendChild(script);
        }

        {
            console.log('FrbExampleGallery.FlutterInitializer add flutter.js script');
            const script = document.createElement("script");
            script.src = '/flutter_rust_bridge/demo/flutter.js';
            // script.defer = true; // TODO
            document.body.appendChild(script);
        }

        window.addEventListener("load", function (ev) {
            console.log('FrbExampleGallery.FlutterInitializer call flutter-loadEntrypoint');
            _flutter.loader.loadEntrypoint({
                onEntrypointLoaded: async function (engineInitializer) {
                    console.log('FrbExampleGallery.FlutterInitializer call flutter-appRunner');
                    let appRunner = await engineInitializer.initializeEngine({
                        // Pass a reference to "div#flutter_host" into the Flutter engine.
                        hostElement: document.querySelector("#flutter_host")
                    });
                    await appRunner.runApp();
                    that.setState({loading: false})
                }
            });
        });
    }

    render() {
        return this.state.loading
            ? <div style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                position: 'absolute',
                top: '0px',
                left: '0px',
                right: '0px',
                bottom: '0px',
            }}>
                <div style={{
                    display: 'flex',
                    flexDirection: 'row',
                    alignItems: 'center',
                }}>
                    <span>Loading...</span>
                    <div style={{
                        marginLeft: '8px',
                        border: '1px solid #f3f3f3',
                        borderTop: '1px solid #2196f3',
                        borderRadius: '50%',
                        width: '12px',
                        height: '12px',
                        animation: 'my-simple-spin 1.2s linear infinite',
                    }}></div>
                </div>
            </div>
            : <></>
    }
}

