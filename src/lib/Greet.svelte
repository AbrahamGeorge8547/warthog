<script lang="ts">
  import { JSONEditor } from "svelte-jsoneditor";
  import { invoke } from "@tauri-apps/api/tauri";

  const themes = [
    { value: "jse-theme-default", label: "default" },
    { value: "jse-theme-dark", label: "dark" },
    { value: "jse-theme-big", label: "big" },
  ];

  const fontSizes = [
    { value: "jse-font-small", label: "small" },
    { value: "jse-font-normal", label: "normal" },
    { value: "jse-font-large", label: "large" },
  ];
  let url = "http://localhost:8000/health";
  let selectedTheme = "jse-theme-dark";
  let selectedFontSize = "jse-theme-normal";
  let types = ["POST", "GET", "PUT", "DELETE"];
  let selectedType = types[0];
  let content = {
    json: {},
  };

  let editorRef;
  function refresh() {
    // call refresh to make sure the line numbers in the gutter are resized too,
    // and the color of the indentation markers is updated
    editorRef?.refresh();
  }

  $: console.log("contents changed:", content);

  const fireRequest = async () => {
    const headers = {
      Accept: "*/*",
      "Access-Control-Allow-Origin": "*",
    };
    const response = await invoke("cors_proxy", {
      url,
      method: selectedType,
      headers,
      body: JSON.stringify(content),
    });
    console.log(response);
    content = {
      json: JSON.parse(response.body),
    };
  };
</script>

<div class="flex p-4 space-x-2">
  <div class="flex-none max-w-5vw">
    <select class="select w-full" bind:value={selectedType}>
      {#each types as type}
        <option value={type}>{type}</option>
      {/each}
    </select>
  </div>

  <div class="flex-grow">
    <input
      type="text"
      class="input flex-grow p-2 rounded-md"
      placeholder="Enter URL"
      bind:value={url}
    />
  </div>
  <button
    type="button"
    class="btn variant-filled-primary rounded-md"
    on:click={fireRequest}>Send</button
  >
</div>

<div class="page {selectedTheme} {selectedFontSize}">
  <div class="editor">
    <JSONEditor bind:content bind:this={editorRef} />
  </div>
</div>

<style lang="scss">
  @import "./jse-theme/jse-theme-dark.css";
  @import "./jse-theme/jse-theme-big.css";

  .page {
    width: 100%;
    height: 100%;
    padding: 10px;
    margin: -10px; // compensate for the padding of the root element
    overflow: auto;

    &.jse-font-small {
      --jse-font-size-mono: 12px;
    }

    &.jse-font-normal {
      --jse-font-size-mono: 14px;
    }

    &.jse-font-large {
      --jse-font-size-mono: 20px;
    }

    .editor {
      width: 90vw;
      height: 50vh;
    }
  }
</style>
