<#macro emailLayout>
<html>
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        body {
            margin: 0;
            padding: 0;
            background-color: #F8F8F8;
            font-family: Arial, sans-serif;
        }
        .container {
            width: 100%;
            background-color: #F8F8F8;
            padding: 20px 0;
        }
        .content {
            max-width: 750px;
            margin: 0 auto;
            background-color: #FFFFFF;
            text-align: left;
        }
        .logo {
            text-align: center;
            margin-bottom: 20px;
            padding-top: 20px;
        }
        .logo img {
            max-width: 40%;
            height: auto;
        }
        .footer {
            margin-top: 20px;
            padding: 10px 0;
            background-color: #F8F8F8;
            color: #777777;
            font-size: 12px;
            text-align: center;
        }
        .footer p {
            margin: 5px 0;
        }
    </style>
</head>
<body>
   <div class="container">
        <div class="content">
            <#nested>
        </div
    </div>
</body>
</html>
</#macro>
