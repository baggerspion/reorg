import Link from 'next/link';
import ReviewCount from './ReviewCount';
import SubmissionCount from './SubmissionCount';

const Conference = (props) => {
    return (
        <tr key={props.conf._id}>
            <td>
                <Link href={`/conferences/${props.conf.short_name}`}><a>{props.conf.name}</a></Link>
            </td>
            <td style={{textAlign: "center"}}><SubmissionCount conf={props.conf._id} /></td>
            <td style={{textAlign: "center"}}><ReviewCount conf={props.conf._id} /></td>
        </tr>
    );
}

export default Conference;