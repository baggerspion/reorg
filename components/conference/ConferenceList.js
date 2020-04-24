import Conference from './Conference'
import Table from 'react-bootstrap/Table'

const ConferenceList = ({data}) => {
    return (
        <Table responsive striped bordered hover size="sm">
            <thead>
                <tr>
                    <th>Event</th>
                    <th style={{textAlign: "center"}}># Submissions</th>
                    <th style={{textAlign: "center"}}># Reviewed</th>
                </tr>
            </thead>
            <tbody>
                {data.allConferences.data.map((event) => (
                    <Conference key={event._id} conf={event} />
                ))}
            </tbody>
        </Table>
    );
}

export default ConferenceList;