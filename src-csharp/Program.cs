using TauriSharp.Services;

WebApplicationBuilder builder = WebApplication.CreateSlimBuilder(args);
builder.Services.AddGrpc();

WebApplication app = builder.Build();

app.MapGrpcService<GreeterService>();

app.Run();