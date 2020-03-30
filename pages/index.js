import fetch from '../components/Fetch'
import Layout from '../components/Layout'
import useSWR from 'swr'

export default function Index() {
  const { data, error } = useSWR('/api/conferences', fetch);
  
  if (error) return <div>failed to load</div>

  return (
    <Layout>
      <div>
        <ul>
        {
        data ? data.conferences.map(conference =>
          <li>{conference.data.title}: {conference.data.start_date}</li>
	      ) : <p>Loading...</p>
        }
	      </ul>
      </div>
    </Layout>
  );
}
