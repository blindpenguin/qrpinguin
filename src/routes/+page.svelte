<script>
  import "../app.css";
  import Navbar from "../libs/Navbar.svelte";
  import FormNew from "../libs/FormNew.svelte";
  import Bottombar from "../libs/Bottombar.svelte";

  /**
   * @type {HTMLDivElement | Bottombar}
   */
  let qrcontainer;

  let thecode;

  function handleQrCodeEvent(event) {
    qrcontainer.innerHTML = "";
    let img = document.createElement("img");
    img.style = "max-height: 100%; margin-left: auto; margin-right: auto;";
    img.src = event.detail.text;
    qrcontainer.appendChild(img);

    thecode = event.detail.text;
  }

  /**
   * @type {FormNew}
   */
  let formNew;

  function handleNewFormEvent() {
    formNew.setActive();
  }
</script>

<div class="flex flex-col h-screen max-h-screen">
  <Navbar on:open-new-form={handleNewFormEvent} />
  <div class="flex flex-grow" style="max-height: calc(100vh - 96px)">
    <div id="codecontainer" class="w-full h-full" bind:this={qrcontainer} />
  </div>
  <div class="flex align-bottom">
    <Bottombar image={thecode} />
  </div>
</div>
<FormNew on:code={handleQrCodeEvent} bind:this={formNew} />

<style>
  :global(body) {
    height: 100vh;
    overflow: hidden;
  }
</style>
