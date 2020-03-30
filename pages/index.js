import fetch from '../components/Fetch'
import Layout from '../components/Layout'
import Link from 'next/link';
import useSWR from 'swr'

const ConfLink = props => (
  <li>
    <Link href="/conference/[id]" as={`/conference/${props.id}`}>
      <a>{props.title}</a>
    </Link>: {props.date}
  </li>
);

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
        <ConfLink id={conference.data.id} title={conference.data.title} date={conference.data.start_date} />
      ) : <p>Loading...</p>
      }
	    </ul>
    </Layout>
  );
}