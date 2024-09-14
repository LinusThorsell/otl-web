<script>
  export let image;
</script>

<div class="bg-cover bg-center" style:background-image={"url(" + image + ")"}>
  <slot />
</div>
