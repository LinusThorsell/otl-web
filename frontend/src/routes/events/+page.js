export async function load({ fetch }) {
  // let res = await fetch(`https://api.teamontheline.com/api/events`)
  let res = await fetch(`http://backend:8000/api/events`)
  let events = await res.json()

  return {
    events: events.body
  };
}
