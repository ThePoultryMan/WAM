<script lang="ts">
  export let element: any;
  $: {
    if (element) lineWidth = element.offsetWidth;
  }
  let lineWidth = 0;
</script>

<hr class="my-1" style="width: {lineWidth * 1.25}px;" />
