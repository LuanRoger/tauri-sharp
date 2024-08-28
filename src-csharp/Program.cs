using TauriSharp.Services;

WebApplicationBuilder builder = WebApplication.CreateSlimBuilder(args);
builder.Logging.ClearProviders();
builder.Services.AddGrpc();

WebApplication app = builder.Build();

app.MapGrpcService<GreeterService>();

app.Run("http://[::1]:5030");