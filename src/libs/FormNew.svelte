<script>
    import { createEventDispatcher } from "svelte";
  import QRCode from "qrcode";
  export const setActive = () => {
    let modal = document.getElementById("modal");
    modal?.classList.remove("hidden");
  };
  export const setInactive = () => {
    let modal = document.getElementById("modal");
    modal?.classList.add("hidden");
  };

  const dispatch = createEventDispatcher();

  let content;

  function createCode() {
    if (content) {
      QRCode.toDataURL(content, {width: "1000"}, function (err, url) {
        dispatch("code", {
            text: url,
        });
        setInactive();
      });
    }
  }
</script>

<div
  id="modal"
  class="relative z-10 hidden"
  aria-labelledby="modal-title"
  role="dialog"
  aria-modal="true"
>
  <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />

  <div class="fixed inset-0 z-10 overflow-y-auto">
    <div
      class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0"
    >
      <div
        class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg"
      >
        <div class="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
          <div class="sm:flex sm:items-start">
            <div class="mt-3 text-center sm:ml-0 sm:mt-0 sm:text-left w-full">
              <h3
                class="text-base font-semibold leading-6 text-gray-900"
                id="modal-title"
              >
                Create new QR Code
              </h3>
              <div class="mt-2">
                <p class="text-sm text-gray-500">
                  Please enter the data in the following field
                </p>
                <textarea bind:value={content} class="border w-full" rows="5" />
              </div>
            </div>
          </div>
        </div>
        <div class="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
          <button
            type="button"
            class="inline-flex w-full justify-center rounded-md bg-green-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-green-500 sm:ml-3 sm:w-auto"
            on:click={createCode}>Create now</button
          >
          <button
            type="button"
            class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:mt-0 sm:w-auto"
            on:click={setInactive}>Cancel</button
          >
        </div>
      </div>
    </div>
  </div>
</div>
