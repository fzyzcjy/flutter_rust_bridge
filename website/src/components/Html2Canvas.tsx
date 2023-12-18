import html2canvas from "html2canvas";
import React from 'react';

// ref: https://blog.logrocket.com/export-react-components-as-images-html2canvas/
// noinspection JSUnusedGlobalSymbols
export default function Html2Canvas({children}) {
    const exportRef = React.useRef();
    return <>
        <button onClick={() => exportAsImage({
            element: exportRef.current,
            imageFileName: "screenshot",
        })}>
            Take screenshot
        </button>
        <div ref={exportRef}>{children}</div>
    </>
}

const exportAsImage = async ({element, imageFileName}) => {
    const canvas = await html2canvas(element);
    const image = canvas.toDataURL("image/png", 1.0);
    downloadImage(image, imageFileName);
};
const downloadImage = (blob, fileName) => {
    const fakeLink = window.document.createElement("a");
    fakeLink.download = fileName;
    fakeLink.href = blob;
    document.body.appendChild(fakeLink);
    fakeLink.click();
    document.body.removeChild(fakeLink);
    fakeLink.remove();
};
