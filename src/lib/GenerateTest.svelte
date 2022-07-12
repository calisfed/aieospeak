<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import stop from "../../sources/stop.mp4";
  const url1 = "https://raw.githubusercontent.com/calisfed/aieospeak/main/sources/part1.txt"
  const url2 = "https://raw.githubusercontent.com/calisfed/aieospeak/main/sources/part2.txt"

  let path1,path2;
  fetch(url1).then((response) => response.blob() ).then(result => result.text()).then(re => path1 = re)
  fetch(url2).then((response) => response.blob() ).then(result => result.text()).then(re => path2 = re)

  let tmp = "";
  let part1 = "";
  let part2 = "";
  let part3 = "";
  let mode = false;
  let start = false;
  let time4stop = false;
  const gen_part1 = () => {
    invoke("gen_test", {
      path: path1,
      Ret: [part1, tmp],
    }).then((response) => ([part1, tmp] = response));
    console.log(path1)

  };
  const gen_part2 = () => {
    invoke("gen_test", {
      path: path2,
      Ret: [part2, part3],
    }).then((response) => ([part2, part3] = response));
    console.log(path2)

  };
  const gen = () => {
    gen_part1();
    gen_part2();
  };
  const test_mode = () => {
    if (mode == false){
      mode = true;
      start = false;
      time4stop = false;
    }
    else {
      mode = false
    }
  };
  const start_mode = () => {
    invoke("sound", { start: start }).then(
      (response) => (time4stop = response)
    );
    start = start ? false : true;
  };
</script>

<button on:click={gen}>Generate</button>
<button on:click={gen_part1}>Part 1</button>
<button on:click={gen_part2}>Part 2-3</button>
<button on:click={test_mode}>Test mode: {mode}</button>
<p />

{#if mode == true}
  <p />
  <button on:click={start_mode}>Start: {start} - {time4stop}</button>
  {#if start == true && time4stop == false}
    <p />
    <div class="part">PART 2</div>
    <div class="test">
      {part2}
    </div>
  {:else if start == true && time4stop == true}
    <video src={stop} autoplay width="720" height="480">
      <track kind="captions" />
    </video>
  {:else}
    <p />
  {/if}
{/if}

{#if mode == false}
  <div class="part">PART 1</div>
  <div class="test">
    {part1}
  </div>
  <p />

  <div class="part">PART 2</div>
  <div class="test">
    {part2}
  </div>
  <p />

  <div class="part">PART 3</div>
  <div class="test">
    {part3}
  </div>
  <p />
{/if}

<style>
  video {
    align-self: center;
  }
  .part {
    color: brown;
    text-transform: uppercase;
    font-size: 1.5rem;
    line-height: 1.1;
    font-weight: 500;
  }
  .test {
    display: flex;
    text-align: left;
    white-space: pre-wrap;
  }
  button {
    font-family: inherit;
    font-size: inherit;
    padding: 0.5em 0.5em;
    color: #ff3e00;
    background-color: rgba(255, 62, 0, 0.1);
    border-radius: 1em;
    border: 2px solid rgba(255, 62, 0, 0);
    outline: none;
    width: 100;
    font-variant-numeric: tabular-nums;
    cursor: pointer;
  }

  button:focus {
    border: 2px solid #ff3e00;
  }

  button:active {
    background-color: rgba(255, 62, 0, 0.2);
  }
</style>
