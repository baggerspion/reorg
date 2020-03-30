import fetch from '../components/Fetch'
import Layout from '../components/Layout'
import useSWR from 'swr'

export default function Index() {
  const { data, error } = useSWR('/api/conferences', fetch);
  
  if (error) return <div>failed to load</div>

  return (
    <Layout>
      <h2>Welcome to Reorg.</h2>
      <p>This is an experimental system to help manage talk submissions to <a href="https://www.writethedocs.org">Write The Docs</a> events.</p>
      <h3>Conferences</h3>
      <ul>
      {
      data ? data.conferences.map(conference =>
        <li>{conference.data.title}: {conference.data.start_date}</li>
	    ) : <p>Loading...</p>
      }
	    </ul>
    </Layout>
  );
}