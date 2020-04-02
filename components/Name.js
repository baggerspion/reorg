import fetch from './Fetch'
import useSWR from 'swr'

const FullName = props => {
    const { data, error } = useSWR(`/api/user?id=${props.id}`, fetch);
    if (error) return <div>Failed to load conference data!</div>

    return (
        <li>{data ? data.data.first_name + " " + data.data.last_name : "Loading..."}</li>
    );
};

export default FullName;