using System;

namespace LobCorp.Exceptions
{
	public class BadDataPath : Exception
	{
		public BadDataPath()
		{
		}
		public BadDataPath(string message) : base(message)
		{
		}
		public BadDataPath(string message, Exception innerException) : base(message, innerException)
		{
		}
	}
}
