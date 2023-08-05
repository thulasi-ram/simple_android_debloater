<script>
  import { invoke } from '@tauri-apps/api/tauri'
  import { Command } from '@tauri-apps/api/shell'
  import { open } from '@tauri-apps/api/shell';



  let name = ''
  let greetMsg = ''

  async function greet() {
    console.log("call")
    greetMsg = await invoke('greet', { name })
  }

  let devices = ''
  let devicesErr = ''
  async function adb_list_devices() {
    await open('https://github.com/tauri-apps/tauri');

    // const cmd = new Command('adb-devices');
    // const cmdOutput = await cmd.execute();


    // console.log(cmdOutput.code)
    // console.log(cmdOutput.signal)
    // console.log(cmdOutput.stderr)
    // console.log(cmdOutput.stdout)
    // const output = await new Command('echo', 'message').execute();
    // console.log(output.code)
    // console.log(output.signal)
    // console.log(output.stderr)
    // console.log(output.stdout)
    devices = "test"
    devicesErr = "test Err"
  }
</script>

<div class="space-y-12">
  <input id="greet-input" placeholder="Enter a name..." bind:value="{name}" />
  <button on:click="{greet}">Greet</button>
  <p>{greetMsg}</p>

  <button on:click="{adb_list_devices}">ADB List</button>
  <p>{devices}</p>
  <p>{devicesErr}</p>

</div>
