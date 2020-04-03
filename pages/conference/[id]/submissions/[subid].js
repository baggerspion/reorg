import fetch from '../../../../components/Fetch'
import FullName from '../../../../components/Name'
import Layout from '../../../../components/Layout'
import { useRouter } from 'next/router';
import useSWR from 'swr'

const Scores = props => {
    const rows = props.scores.map((element) =>
        <tr>
            <td><FullName id={element.reviewer} /></td>
            <td>{element.score}</td>
            <td>{element.notes}</td>
        </tr>
    )

    return (
        <table>
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Score</th>
                    <th>Comment</th>
                </tr>
            </thead>
            <tbody>
                {rows}
            </tbody>
        </table>
    )
}

export default function Submission() {
    const router = useRouter()
    const { data, error } = useSWR("/api/submission?id=" + router.query.subid, fetch)

    if (error) return <div>Error loading submission data</div>
    if (!data) return <div>Loading...</div>

    return (
            <Layout>
            <h2>{data.data.title}</h2>
            <h4>Submission By</h4>
            <ul>{data.data.author.map((author) => <li><FullName id={author} /></li>)}</ul>
            <h4>Submission Date/Time</h4>
            <p>Date/Time of submission goes here. Needs to be stored!</p>
            <h4>Abstract</h4>
            <p>{data.data.abstract}</p>
            <hr />
            <h3>Scores</h3>
            <Scores scores={data.data.reviews} />
        </Layout>
    )
}