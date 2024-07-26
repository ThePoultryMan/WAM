<script>
  import "../app.css";

  import PageStyledLink from "$components/PageStyledLink.svelte";
</script>

<div class="flex">
  <div class="leading-none *:block *:pl-3 *:pr-4 *:py-1.5">
    <PageStyledLink href="/" class="bg-blue-400">Play</PageStyledLink>
    <PageStyledLink href="/info" class="bg-blue-400">Info</PageStyledLink>
    <PageStyledLink href="/addons" class="bg-blue-400">Addons</PageStyledLink>
  </div>
  <div class="mx-5 mt-2">
    <slot></slot>
  </div>
</div>
