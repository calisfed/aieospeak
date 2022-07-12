<script>
  import GenerateTest from "./lib/GenerateTest.svelte";
</script>

<main>
  <h1>IELTS Speaking generator!</h1>
  <div>Forecasting topics - Season 2 - 2022</div>
  <p></p>
  
  
  <GenerateTest />

  <div></div>
  <a href="https://github.com/calisfed/l">@Calisfed</a>
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }

  a {
    position: static;
    bottom: 10px;
    right: 10px;
    font-size: xx-small;
  }

  main {
    text-align: center;
    padding: 1em;
    margin: 0 auto;
    justify-content: space-around;
    flex-direction: row;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 2rem;
    line-height: 1.1;
    margin: 2rem auto;
    max-width: 14rem;
  }


  @media (min-width: 480px) {
    h1 {
      max-width: none;
    }
  }
</style>
