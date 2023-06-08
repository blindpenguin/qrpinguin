<script>
  import { createEventDispatcher } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  const dispatch = createEventDispatcher();

  function openNewForm() {
    dispatch("open-new-form");
  }

  async function quitApp() {
    await invoke("quit_app");
  }
</script>

<ul class="flex items-space-between w-full bg-slate-100" on:click={openNewForm} on:keyup>
  <li class="p-3 hover:bg-slate-200 hover:cursor-pointer" title="Create new">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
      class="w-6 h-6"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M12 4.5v15m7.5-7.5h-15"
      />
    </svg>
  </li>
  <li class="grow"></li>
  <li class="p-3 bg-red-500 text-white hover:bg-red-700 hover:cursor-pointer" on:click={quitApp} on:keyup title="Quit application">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
      class="w-6 h-6"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M5.636 5.636a9 9 0 1012.728 0M12 3v9"
      />
    </svg>
  </li>
</ul>
