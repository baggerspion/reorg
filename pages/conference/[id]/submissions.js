import fetch from '../../../components/Fetch'
import FullName from '../../../components/Name'
import Layout from '../../../components/Layout'
import { useRouter } from 'next/router';
import useSWR from 'swr'
import Score from '../../../components/Score'

const Submission = props => {
    const { data, error } = useSWR("/api/submissions?id=" + props.id, fetch);
    if (error) return <tr><td colSpan="7">Error loading submission details.</td></tr>
    if (!data) return <tr><td colSpan="7">Loading submissions...</td></tr>

    return (
        data.map((element) =>
            <tr>
                <td><ul>{element.data.author.map((name) => <FullName id={name} />)}</ul></td>
                <td>{element.data.title}</td>
                <td></td>
                <td><Score data={element.data.reviews} /></td>
                <td></td>
                <td>❌</td>
                <td></td>
            </tr>
        )
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
            <table>
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Title</th>
                        <th>Your Review</th>
                        <th>Score</th>
                        <th>Allocated</th>
                        <th></th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    <Submission id={data['ref']['@ref']['id']} />
                </tbody>
            </table>
        </Layout>
    )
}