import init, { pixel_filter } from './rust_videofilter.js';

const video = document.getElementById('myVideo');
const canvas = document.getElementById('myCanvas');
const ctx = canvas.getContext('2d',{willReadFrequently: true,});
const dot_size = document.getElementById("dot_size");

navigator.mediaDevices.getUserMedia({ video: true, audio: false })
    .then(stream => {
        video.srcObject = stream;
        // 描画を開始
        video.addEventListener('loadeddata', () => {
            canvas.width = video.videoWidth;
            canvas.height = video.videoHeight;
            function draw() {
                ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
                requestAnimationFrame(draw);
            }
            draw();
        });
    })
    .catch(err => {
        console.error('エラー:', err);
    });

async function run() {
    await init();
    document.getElementById("pixel_filter").addEventListener("click", () => {
        canvas.width = video.videoWidth;
        canvas.height = video.videoHeight;
        function draw() {
            const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
            const ret = pixel_filter(new Uint8Array(imageData.data.buffer),canvas.width,canvas.height,dot_size.value);
            ctx.putImageData(new ImageData(new Uint8ClampedArray(ret.buffer), canvas.width, canvas.height), 0, 0);
            requestAnimationFrame(draw);
        }
        draw();
    });
    document.getElementById("none_filter").addEventListener("click", () => {
        canvas.width = video.videoWidth;
        canvas.height = video.videoHeight;
        function draw() {
            ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
            requestAnimationFrame(draw);
        }
        draw();
    });
}
run();
