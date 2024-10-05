<script>
  import { Input, Select, Label, Fileupload, Button, Accordion, AccordionItem } from 'flowbite-svelte';
  import Container from '$lib/Container.svelte';
  import CreateEventForm from '$lib/admin/CreateEventForm.svelte';
  import CreateTourForm from '$lib/admin/CreateTourForm.svelte';
  import { PUBLIC_BACKEND_URL } from '$env/static/public';

  export let data;

  let apikey = '';
</script>

<Container>
  <div>
    <Label for="apikey" class="block mb-2">API-key</Label>
    <Input bind:value={apikey} id="apikey" type="password" placeholder="Enter apikey" />
  </div>

  <Accordion flush class="w-full" activeClass="bg-blue-100 dark:bg-gray-800 text-blue-600 dark:text-white focus:ring-4 focus:ring-blue-200 dark:focus:ring-blue-800" inactiveClass="text-gray-500 dark:text-gray-400 hover:bg-gray-700 dark:hover:bg-gray-800">
    <AccordionItem class="w-full">
      <span slot="header">Create tour</span>
      <CreateTourForm apikey={apikey} />
    </AccordionItem>
    <AccordionItem class="w-full">
      <span slot="header">Create event</span>
      <CreateEventForm apikey={apikey} tours={data.tours} />
    </AccordionItem>
  </Accordion>
</Container>
