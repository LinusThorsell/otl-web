import { PUBLIC_BACKEND_URL } from '$env/static/public';
export async function load({ fetch }) {
  let tours_res = await fetch(PUBLIC_BACKEND_URL + '/api/tours')
  let tours = await tours_res.json()

  let events_res = await fetch(PUBLIC_BACKEND_URL + '/api/events')
  let events = await events_res.json()

  return {
    tours: tours.body,
    events: events.body
  };
}
