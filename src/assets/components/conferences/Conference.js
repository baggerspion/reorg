import { NavLink } from 'react-router-dom';
import React, {Component} from 'react';
import SubmissionCount from './SubmissionCount';

class Conference extends Component {
    render() {
        return (
            <tr key={this.props.conf._id}>
                <td>
                    <NavLink exact to={`/conferences/${this.props.conf.short_name}`}>{this.props.conf.name}</NavLink>
                </td>
                <td style={{textAlign: "center"}}><SubmissionCount event={this.props.conf._id} /></td>
                <td style={{textAlign: "center"}}>8</td>
            </tr>
        );
    }
}

export default Conference;