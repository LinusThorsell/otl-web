export async function load({ fetch }) {
  let res = await fetch(`https://api.teamontheline.com/api/events`)
  let events = await res.json()

  return {
    events: events.body
  };
}
