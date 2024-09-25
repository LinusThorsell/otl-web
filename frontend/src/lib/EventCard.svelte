<script>
  import { Card, Button } from 'flowbite-svelte';
  import { ArrowRightOutline } from 'flowbite-svelte-icons';

  export let event;
  export let button;
  export let url;
  export let image;
</script>

<Card class="mt-4 sm:mx-2">
  <img src={image} alt="Cover" class="mb-2 rounded-lg" />
  {#if event.start_date}
    <p>{new Date(event.start_date).toLocaleDateString('sv-SE')} - {new Date(event.end_date).toLocaleDateString('sv-SE')}</p>
  {:else}
    <p>{new Date(event.date).toLocaleDateString('sv-SE')} {new Date(event.date).toLocaleTimeString('sv-SE')}</p>
  {/if}
  <h1 class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{event.title}</h1>
  <h2 class="mb-2">{event.description}</h2>

  {#if event.body}
    <p class="mb-2 font-normal text-gray-700 dark:text-gray-400 leading-tight">{event.body}</p>
  {/if}
  <Button href={url ? url : event.url} class="w-full mt-auto shadow">
    {button}
  </Button>
</Card>


