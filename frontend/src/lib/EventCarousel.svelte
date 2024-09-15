<script>
  import { Button, Card } from 'flowbite-svelte';

  import EventCard from './EventCard.svelte';

  export let events;
  export let eventsPerSlide = 3;
  if (eventsPerSlide == undefined) {
    eventsPerSlide = events.length;
  }

  let displayedEvents = events.slice(0, eventsPerSlide);
  let showMore = events.length > eventsPerSlide;

  let size;
</script>
<svelte:window bind:innerWidth={size} />

<div class="flex flex-wrap justify-center lg:w-5/6 w-11/12 pb-4">
  {#each displayedEvents as event}
    <EventCard event={event} button="Registrera dig via Tjing" url={event.url} />
  {/each}
  {#if size < 768}
    {#if showMore}
      <Card class="mt-4">
        <Button
          on:click={() => {
            displayedEvents = events;
            showMore = false;
          }}
          class="w-full"
        >
          Visa alla framtida evenemang
        </Button>
      </Card>
    {/if}
  {/if}
</div>

