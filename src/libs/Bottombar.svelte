<script>
  import { save } from "@tauri-apps/api/dialog";
  import { BaseDirectory, writeBinaryFile } from "@tauri-apps/api/fs";
  import { homeDir } from '@tauri-apps/api/path';

  export let image;

  const saveQRCode = async () => {
    const suggestedFilename = "qrcode.png";
    const baseDirectory = await homeDir();

    const filePath = await save({
      defaultPath: (baseDirectory + "/" + suggestedFilename),
      filters: [
        {
          name: "Image",
          extensions: ["png"],
        },
      ],
    });

    const binaryData = atob(image.split(",")[1]);

    const byteArray = new Uint8Array(binaryData.length);
    for (let i = 0; i < binaryData.length; i++) {
      byteArray[i] = binaryData.charCodeAt(i);
    }

    await writeBinaryFile(filePath, byteArray);
  };

</script>

<ul class="flex items-space-between w-full bg-slate-100">
  <li
    class="p-3 hover:bg-slate-200 hover:cursor-pointer"
    title="Create new"
    on:click={saveQRCode}
    on:keyup
  >
    Save QR Code
  </li>
  <li class="grow" />
</ul>
