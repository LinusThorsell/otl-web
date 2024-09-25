<script>
  import { Input, Label, Fileupload, Button, Accordion, AccordionItem } from 'flowbite-svelte';
  import Container from '$lib/Container.svelte';
  import { PUBLIC_BACKEND_URL } from '$env/static/public';

  let apikey = '';
  let title = '';
  let date = '';
  let url = '';
  let image = null;

  function handleFileUpload(event) {
    image = event.target.files[0]; // Grab the first file from the FileList
  }

  async function submitForm(event) {
    event.preventDefault();

    const formData = new FormData();
    formData.append('title', title);
    formData.append('date', date);
    formData.append('url', url);

    if (image) {
      formData.append('image', image); // Append the actual file
    }

    try {
      const response = await fetch(`${PUBLIC_BACKEND_URL}/api/event/create`, {
        method: 'POST',
        headers: {
          'X-Api-Key': apikey,
        },
        body: formData,
      });

      if (!response.ok) {
        throw new Error('Failed to submit form');
      }

      // Handle successful submission
      console.log('Form submitted successfully');
    } catch (error) {
      console.error('Error submitting form:', error);
    }
  }
</script>

<Container>
  <div>
    <Label for="apikey" class="block mb-2">API-key</Label>
    <Input bind:value={apikey} id="apikey" type="password" placeholder="Enter apikey" />
  </div>

  <Accordion flush class="w-full" activeClass="bg-blue-100 dark:bg-gray-800 text-blue-600 dark:text-white focus:ring-4 focus:ring-blue-200 dark:focus:ring-blue-800" inactiveClass="text-gray-500 dark:text-gray-400 hover:bg-gray-700 dark:hover:bg-gray-800">
    <AccordionItem class="w-full">
      <span slot="header">Create tour</span>
      <p>Coming soon!</p>
    </AccordionItem>
    <AccordionItem class="w-full" open>
      <span slot="header">Create event</span>
      <form on:submit={submitForm}>
        <div class="mb-2">
          <Label for="title" class="mb-2">Title</Label>
          <Input bind:value={title} type="text" id="title" required />
        </div>
        <div class="mb-2">
          <Label for="date" class="mb-2">Datetime</Label>
          <Input bind:value={date} type="datetime-local" id="date" required />
        </div>
        <div class="mb-2">
          <Label for="url" class="mb-2">Signup URL</Label>
          <Input bind:value={url} type="text" id="url" required />
        </div>
        <div class="mb-2">
          <Label for="image" class="mb-2">Cover image</Label>
          <Fileupload id="image" on:change={handleFileUpload} class="mb-2" />
        </div>
        <Button type="submit">Submit</Button>
      </form>
    </AccordionItem>
  </Accordion>
</Container>
