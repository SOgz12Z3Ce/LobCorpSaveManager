using System;

namespace LobCorp.Exceptions
{
	public class BadArguments : Exception
	{
		public BadArguments()
		{
		}
		public BadArguments(string message) : base(message)
		{
		}
		public BadArguments(string message, Exception innerException) : base(message, innerException)
		{
		}
	}
}
