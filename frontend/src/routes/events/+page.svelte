<script>
  import { Button } from 'flowbite-svelte';
  import { ArrowRightOutline } from 'flowbite-svelte-icons';
  import EventCarousel from '$lib/EventCarousel.svelte';

  export let data;
</script>

<div class="flex flex-wrap justify-center">
  <EventCarousel events={data.events} />
</div>
<div class="flex justify-center">
  <Button href="/results" class="w-fit">
    Se resultat för tidigare tävlingar
  </Button>
</div>
