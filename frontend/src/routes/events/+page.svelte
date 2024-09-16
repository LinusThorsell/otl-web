<script>
  import { Button } from 'flowbite-svelte';
  import { ArrowRightOutline } from 'flowbite-svelte-icons';
  import EventCarousel from '$lib/EventCarousel.svelte';

  export let data;
</script>

<div class="mt-2 flex justify-center">
  <Button href="/results" class="w-fit">
    Se resultat för tidigare tävlingar
  </Button>
</div>
<div class="flex flex-wrap justify-center">
  <EventCarousel events={data.events} />
</div>
