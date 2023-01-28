<script>
  import { dev } from "$app/environment";
  /**
   * @type {string}
   */
  export let url;
  if (dev) {
    url = "http://localhost:5173/";
  } else {
    url = "https://priede-editor.vercel.app/";
  }
</script>

<div class="root">
  <header>
    <h1>Priede</h1>
    <h2>Latviešu valodā rakstāma programmēšanas valoda</h2>
  </header>
  <main>
    <h2>Sveika pasaule piemērs</h2>
    <!-- svelte-ignore a11y-missing-attribute -->
    <iframe src={url} frameborder="0" />
    <h2>Mainīgie</h2>
    <p>Mainīgos definē:</p>

    <strong>TIPS NOSAUKUMS -> VĒRTĪBA</strong>

    <p>Datu tipi:</p>
    <ul>
      <li>būls - divas iespējamas vērtības PATIESS vai NEPATIESS</li>
      <li>sk - 32 bitu vesels skaitlis</li>
      <li>lsk - 64 bitu vesels skaitlis</li>
      <li>natsk - 32 bitu naturāls skaitlis</li>
      <li>lnatsk - 64 bitu naturāls skaitlis</li>
      <li>teksts - teksts</li>
    </ul>
    <!-- svelte-ignore a11y-missing-attribute -->
    <iframe src={url + "?preset=vars"} frameborder="0" />
  </main>
  <footer>
    <a href="https://github.com/MarcisAn/priede">GITHUB</a>
  </footer>
</div>

<style>
  iframe {
    width: 100%;
    height: 20em;
  }

  .root {
    display: flex;
    flex-direction: column;
    height: 100%;
    background-color: rgb(44, 45, 46);
  }
  header {
    background-color: rgb(255, 225, 92);
  }
  footer {
    background-color: rgb(255, 225, 92);
    padding: 1em;
  }
  main {
    flex: 1;
    color: white;
  }
</style>
