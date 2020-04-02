import fetch from '../../../components/Fetch'
import Layout from '../../../components/Layout'
import { useRouter } from 'next/router';
import useSWR from 'swr'

const Submission = props => {
    const { data, error } = useSWR("/api/submissions?id=" + props.id, fetch);
    if (error) return <div>Error loading submission details.</div>
    if (!data) return <div>Loading submissions...</div>

    const items = data.map((element) =>
        <li>{element.data.title}</li>
    );

    return (
       <ul>
           {items}
       </ul>
    )
};

export default function Conference() {
    const router = useRouter();
    const { data, error } = useSWR("/api/conference?conf=" + router.query.id, fetch);

    if (error) return <div>Error loading conference details.</div>
    if (!data) return <div>Loading...</div>

    return (
        <Layout>
            <h2>Submissions</h2>
            <Submission id={data['ref']['@ref']['id']} />
        </Layout>
    )
}