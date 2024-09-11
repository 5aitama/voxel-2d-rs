<script lang="ts">
    import Example from "$lib/example.svelte";
    import "../app.css";
</script>

<div class="title">
    <h1>Voxel 2D 🦀</h1>
    <h3>Quick demonstrations</h3>
</div>

<div class="content">
    <Example />
</div>

<style>
    .title {
        position: relative;
        padding-left: 20px;
        margin-bottom: 20px;
    }

    .title::after {
        display: block;
        position: absolute;

        content: "";

        width: 3px;
        height: 100%;

        top: 0;
        left: 0;

        border-radius: 2px;

        background: var(--color);
    }

    .content {
        width: 100%;
        height: 100%;
    }
</style>
