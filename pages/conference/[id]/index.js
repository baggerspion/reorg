import fetch from '../../../components/Fetch'
import FullName from '../../../components/Name'
import Layout from '../../../components/Layout';
import Link from 'next/link'
import { useRouter } from 'next/router';
import useSWR from 'swr'

const Submissions = props => {
    const router = useRouter();
    const { data, error } = useSWR(`/api/submissions?id=${props.id}`, fetch);
    if (error) return <div>Failed to load submissions data!</div>
    if (!data) return <div>Loading...</div>

    return (
        <>
        <p>{data.length > 0 ? <b>{data.length}</b> : "No"} {data.length == 1 ? "submission" : "submissions"} received.</p>
        {data.length > 0 ? <p><Link href={"/conference/" + router.query.id + "/submissions"}><a>View Submissions</a></Link></p> : ""}
        </>
    );
};

export default function Conference() {
    const router = useRouter();
    const { data, error } = useSWR(`/api/conference?conf=${router.query.id}`, fetch);
    
    if (error) return <div>Failed to load conference data!</div>
    if (!data) return <div>Loading...</div>

    return (
        <Layout>
            <h2>{data.data.title}: {data.data.start_date}</h2>
            <h3>Submissions</h3>
            <Submissions id={data['ref']['@ref']['id']} />
            <h3>Resources</h3>
            <h3>Reviewers</h3>
            <ul>
            {
                data.data.reviewers.map((name) => <li><FullName id={name} /></li>)
            }
            </ul>
        </Layout>
    );
}