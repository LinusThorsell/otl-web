<script>
  import { Input, Select, Label, Fileupload, Button } from 'flowbite-svelte';
  import { handleFormSubmit } from '$lib/admin/formHandler.js';

  export let apikey;
  export let events;

  const available_events = events.map((event) => ({ value: event.id, name: event.title }));

  let event_id = null;
  let score = null;

  function handleFileUpload(event) {
    score = event.target.files[0];
  }

  async function submitForm(event) {
    event.preventDefault();

    const formData = new FormData();
    if (score) formData.append('file', score);

    handleFormSubmit(formData, `/api/event/ingest/${event_id}`, apikey)
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
      <Label for="event_id" class="mb-2">Event</Label>
      <Select id="event_id" items={available_events} bind:value={event_id} />
    </div>
    <div class="mb-2">
      <Label for="score" class="mb-2">Score file</Label>
      <Fileupload id="score" on:change={handleFileUpload} class="mb-2" />
    </div>
    <Button type="submit">Submit</Button>
  </form>
</div>
