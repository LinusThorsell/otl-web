<script>
  import { Card, Button } from 'flowbite-svelte';
  import { MapPinAltSolid } from 'flowbite-svelte-icons';
  import { PUBLIC_BACKEND_URL } from '$env/static/public';

  export let event;
  export let button;
  export let url;

  let image = PUBLIC_BACKEND_URL + "/static/uploads/" + event.image;
</script>

<Card class="mt-4 sm:mx-2">
  <div class="flex items-center mb-2" style="margin-top: -0.75rem;"><MapPinAltSolid class="mr-2" /> <span class="text-xl">{event.location}</span></div>

  <div class="mb-2 rounded-lg overflow-hidden" style="position: relative; padding-top: 56.25%;">
    <img src={image} alt="Cover" class="absolute top-0 left-0 w-full h-full" style="object-fit: cover" />
  </div>


  {#if event.start_date}
    <p>{new Date(event.start_date).toLocaleDateString('sv-SE')} - {new Date(event.end_date).toLocaleDateString('sv-SE')}</p>
  {:else}
    <p>{new Date(event.date).toLocaleDateString('sv-SE')} {new Date(event.date).toLocaleTimeString('sv-SE')}</p>
  {/if}

  <h1 class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{event.title}</h1>
  <h2 class="mb-2">{event.description}</h2>

  <Button href={url ? url : event.url} class="w-full mt-auto shadow">
    {button}
  </Button>
</Card>


