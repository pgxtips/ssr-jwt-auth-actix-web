async function handleLogin(e){
    console.log('Login button clicked!');
    e.preventDefault();

    const formData = new FormData(e.target);

    const username = formData.get('username');   
    const password = formData.get('password');

    const data = {
        username: username,
        password: password
    }

    let res = await fetch('/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    });

    let json = await res.json();
    try{
        let jwt_bearer = json.token;
        //set browser to use token for future requests as bearer token in authorization header
        localStorage.setItem('jwt', jwt_bearer);
        console.log("Login successful, token received");
        window.location.href = '/dashboard';
    }catch(e){
        console.log("Login failed, no token received");
    } 
}


function removeJWT(){
    localStorage.removeItem('jwt');
}
