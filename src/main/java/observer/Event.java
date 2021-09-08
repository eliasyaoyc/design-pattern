package observer;

/**
 * @author Elias (siran0611@gmail.com)
 */
public class Event {
	private String user;
	private String content;

	public Event(String user, String content) {
		this.user = user;
		this.content = content;
	}

	public String getUser() {
		return user;
	}

	public void setUser(String user) {
		this.user = user;
	}

	public String getContent() {
		return content;
	}

	public void setContent(String content) {
		this.content = content;
	}
}
