import { PUBLIC_BACKEND_URL } from '$env/static/public';
export async function load({ fetch }) {
  let res = await fetch(PUBLIC_BACKEND_URL + '/api/events')
  let events = await res.json()

  return {
    events: events.body
  };
}
