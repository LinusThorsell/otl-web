<script>
  import { Input, Select, Label, Fileupload, Button } from 'flowbite-svelte';
  import { handleFormSubmit } from '$lib/admin/formHandler.js';

  export let apikey;
  export let tours;

  const available_tours = tours.map((tour) => ({ value: tour.id, name: tour.title }));

  let title = '';
  let location = '';
  let description = '';
  let body = '';
  let date = '';
  let url = '';
  let image = null;
  let tour = null;

  function handleFileUpload(event) {
    image = event.target.files[0];
  }

  async function submitForm(event) {
    event.preventDefault();

    const formData = new FormData();
    formData.append('title', title);
    formData.append('location', location);
    formData.append('description', description);
    formData.append('body', body);
    formData.append('date', date);
    formData.append('url', url);
    if (image) formData.append('image', image);
    if (tour) formData.append('tour_id', tour);

    handleFormSubmit(formData, "/api/event/create", apikey)
      .then(response => {
        console.log('Form submitted successfully with response:', response);
      })
      .catch(error => {
        console.error('Error:', error);
      });
  }
</script>

<div>
  <form on:submit={submitForm}>
    <div class="mb-2">
      <Label for="title" class="mb-2">Title</Label>
      <Input bind:value={title} type="text" id="title" required />
    </div>
    <div class="mb-2">
      <Label for="location" class="mb-2">Location</Label>
      <Input bind:value={location} type="text" id="location" required />
    </div>
    <div class="mb-2">
      <Label for="description" class="mb-2">Description</Label>
      <Input bind:value={description} type="text" id="description" required />
    </div>
    <div class="mb-2">
      <Label for="body" class="mb-2">Body</Label>
      <Input bind:value={body} type="text" id="body" required />
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
    <div class="mb-2">
      <Label for="tour_id" class="mb-2">Tour</Label>
      <Select id="tour_id" items={available_tours} bind:value={tour} />
    </div>
    <Button type="submit">Submit</Button>
  </form>
</div>
