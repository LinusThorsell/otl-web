<script>
  import { Input, Select, Label, Fileupload, Button } from 'flowbite-svelte';
  import { handleFormSubmit } from '$lib/admin/formHandler.js';

  export let apikey;

  let title = '';
  let location = '';
  let description = '';
  let body = '';
  let start_date = '';
  let end_date = '';
  let url = '';
  let score_count = 0;
  let image = null;

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
    formData.append('start_date', start_date);
    formData.append('end_date', end_date);
    formData.append('url', url);
    formData.append('score_count', score_count);
    if (image) formData.append('image', image);

    handleFormSubmit(formData, "/api/tour", apikey)
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
      <Label for="start_date" class="mb-2">Start Datetime</Label>
      <Input bind:value={start_date} type="datetime-local" id="start_date" required />
    </div>
    <div class="mb-2">
      <Label for="end_date" class="mb-2">End Datetime</Label>
      <Input bind:value={end_date} type="datetime-local" id="end_date" required />
    </div>
    <div class="mb-2">
      <Label for="url" class="mb-2">Signup URL</Label>
      <Input bind:value={url} type="text" id="url" required />
    </div>
    <div class="mb-2">
      <Label for="score_count" class="mb-2">Events to count</Label>
      <Input bind:value={score_count} type="text" id="score_count" required />
    </div>
    <div class="mb-2">
      <Label for="image" class="mb-2">Cover image</Label>
      <Fileupload id="image" on:change={handleFileUpload} class="mb-2" />
    </div>
    <Button type="submit">Submit</Button>
  </form>
</div>
