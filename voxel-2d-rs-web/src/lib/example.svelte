<script lang="ts">
    import { onMount } from "svelte";

    let canvas: HTMLCanvasElement;
    let mouse_x = 1.0;
    let mouse_y = 1.0;

    function onMouseMove(ev: MouseEvent) {
        const bounds = canvas.getBoundingClientRect();
        const x = Math.round(
            Math.max(Math.min(ev.clientX - bounds.x, bounds.width), 0),
        );

        const y = Math.round(
            Math.max(
                Math.min(
                    bounds.height - (ev.clientY - bounds.y),
                    bounds.height,
                ),
                0,
            ),
        );

        mouse_x = x * devicePixelRatio;
        mouse_y = y * devicePixelRatio;
    }

    onMount(() => {
        window.addEventListener("mousemove", onMouseMove);

        import("./voxel-2d-rs").then(async (module) => {
            await module.default();

            const t = new module.Test();
            const render = () => {
                t.render(0.01, 0.01, mouse_x, mouse_y);
                requestAnimationFrame(render);
            };

            render();
        });

        return () => {
            window.removeEventListener("mousemove", onMouseMove);
        };
    });
</script>

<canvas bind:this={canvas} id="canvas"></canvas>

<style>
    canvas {
        background: rgb(250, 250, 250);

        width: 100%;
        height: 100%;

        display: block;

        border: 1px solid rgb(240, 240, 240);
        box-sizing: border-box;
    }
</style>
