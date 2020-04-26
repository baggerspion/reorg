import Link from 'next/link';

const Conference = (props) => {
    const reviewed = props.conf.submissions.data.filter(review => review['status'] != "SUBMITTED");

    return (
        <tr key={props.conf._id}>
            <td>
                <Link href={`/conferences/${props.conf._id}`}><a>{props.conf.name}</a></Link>
            </td>
            <td style={{textAlign: "center"}}>{props.conf.submissions.data.length}</td>
            <td style={{textAlign: "center"}}>{reviewed.length}</td>
        </tr>
    );
}

export default Conference;