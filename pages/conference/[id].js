import fetch from '../../components/Fetch'
import Layout from '../../components/Layout';
import { useRouter } from 'next/router';
import useSWR from 'swr'

const Applicants = props => {
    const { data, error } = useSWR(`/api/submissions?id=${props.id}`, fetch);
    if (error) return <div>Failed to load submissions data!</div>
    if (!data) return <div>Loading...</div>

    return (
        <p>{data.length > 0 ? <b>{data.length}</b> : "No"} {data.length == 1 ? "submission" : "submissions"} received.</p>
    );
};

const Reviewer = props => {
    const { data, error } = useSWR(`/api/user?id=${props.id}`, fetch);
    if (error) return <div>Failed to load conference data!</div>

    return (
        <li>{data ? data.data.first_name + " " + data.data.last_name : "Loading..."}</li>
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
            <h3>Applicants</h3>
            <Applicants id={data['ref']['@ref']['id']} />
            <h3>Resources</h3>
            <h3>Reviewers</h3>
            <ul>
            {
            data.data.reviewers ? (
                data.data.reviewers.map(rev => <Reviewer id={rev} />
            )) : ( <li>This conference has no reviewers.</li> )
            }
            </ul>
        </Layout>
    );
}