import fetch from '../../components/Fetch'
import Layout from '../../components/Layout';
import { useRouter } from 'next/router';
import useSWR from 'swr'

export default function Conference() {
    const router = useRouter();
    const { data, error } = useSWR(`/api/conference?conf=${router.query.id}`, fetch);
    
    if (error) return <div>Failed to load conference data!</div>
    if (!data) return <div>Loading...</div>

    return (
        <Layout>
            <h2>{data.data.title}: {data.data.start_date}</h2>
            <h3>Applicants</h3>
            <h3>Resources</h3>
            <h3>Reviewers</h3>
        </Layout>
    );
}