import init, { dot_art_generate } from './rust_dotart.js';

async function run() {
    await init();

    document.querySelectorAll("[data-dot]").forEach(ele =>{
        ele.addEventListener("click", async (event) => {
            const files = document.getElementById("file_input").files;
            if(files.length === 0)
            {
                window.confirm("err:Fileをセットしてください");
                return;
            }
            const file_blob = new Blob([files[0]], { type: files[0].type });
            await  blobToUint8Array(file_blob)
                .then(uint8Array => {
                    dot_art_generate(event.target.dataset.dot,uint8Array);
                })
                .catch(error => {
                    console.error('Error converting blob:', error);
                });
        });
    })
}
run();


async function blobToUint8Array(blob) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => {
            resolve(new Uint8Array(reader.result));
        };
        reader.onerror = reject;
        reader.readAsArrayBuffer(blob);
    });
}