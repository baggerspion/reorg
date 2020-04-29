import { useState } from "react";
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import Popover from 'react-bootstrap/Popover';

const popover = (
    <Popover id="popover-basic">
        <Popover.Title as="h3">Welcome back to Reorg!</Popover.Title>
        <Popover.Content>
            <Form>
                <Form.Group controlId="formBasicEmail">
                    <Form.Label>Email address</Form.Label>
                    <Form.Control type="email" placeholder="Enter email" />
                    <Form.Text className="text-muted">
                        We'll never share your email with anyone else.
                    </Form.Text>
                </Form.Group>
                <Form.Group controlId="formBasicPassword">
                    <Form.Label>Password</Form.Label>
                    <Form.Control type="password" placeholder="Password" />
                </Form.Group>
                <Form.Group controlId="formBasicCheckbox">
                    <Form.Check type="checkbox" label="Remember me, please!" />
                </Form.Group>
                <Button variant="primary" type="submit">
                    Submit
                </Button>
            </Form>
        </Popover.Content>
    </Popover>
  );

const Login = (props) => {
    const [show, setShow] = useState(false);
    const toggle = () => setPopoverOpen(!popoverOpen);
    
    return (
        <li className="nav-item">
            <OverlayTrigger trigger="click" placement="bottom" overlay={popover}>
                <a className="nav-link">Log In</a>
            </OverlayTrigger>
        </li>
    );
}   

export default Login;